#!/bin/bash
set -eux pipefail

# Declare variables at script level
WEBSITE_REPO=""
BRANCH=""
MAX_DATE=""

init_variables() {
    WEBSITE_REPO="./afbase.github.io"
    BRANCH=$(uuid)
    MAX_DATE=$(date --date='1 day ago' --rfc-3339='date')
}

configure_git() {
    git config --global user.name 'clinton bowen'
    git config --global user.email 'afbase@users.noreply.github.com'
}

create_and_push_branch() {
    git switch -c "${BRANCH}"
    git add .
    git commit -am "Automated california blog update ${MAX_DATE}"
    git push --set-upstream origin "${BRANCH}"
}

create_pull_request() {
    gh pr create \
      --body "${MAX_DATE} automated change" \
      --title "${MAX_DATE} github action water reservoir action" \
      --assignee "@me" \
      --label "automerge" \
      --head=afbase:$(git branch --show-current)
}

main() {
    init_variables
    cd "${WEBSITE_REPO}"
    configure_git
    create_and_push_branch
    create_pull_request
}

main