#!/bin/bash

cd "$(dirname "$0")"

for filename in $(find . -name '*.yml'); do

    echo "Validate ${filename}..."

    errormessage=$(/sbin/modprobe -n -v hfsplus 2>&1)
    error=$(aws cloudformation validate-template --template-body file://"${filename}" 2>&1 | grep error)

    if [[ -n ${error} ]]; then
      echo "${filename} - ${error} - ${errormessage}"
      exit 1
    fi

done

read -p 'What is the build bucket? ' build_bucket
if [[ -z "${build_bucket}" ]]
then
    echo 'ERROR: Build bucket required'
    exit 1
fi

git_branch=$(git branch | grep \* | cut -d ' ' -f2)
stack_suffix=$(echo "${git_branch}" | tr '[:upper:]' '[:lower:]')

echo "Deploying the root CloudFormation stack..."

aws cloudformation deploy \
--template-file ./cfn/pipeline.yml \
--capabilities CAPABILITY_NAMED_IAM \
--stack-name lead-score-"${stack_suffix}"-pipeline \
--parameter-overrides \
GitBranch="${git_branch}" \
BuildBucketName="${build_bucket}"