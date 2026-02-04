#!/bin/bash
set -e

# Configuration
FUNCTION_NAME="band-songbook"
AWS_REGION="eu-west-3"
AWS_ACCOUNT_ID="579871531410"
ECR_REPO="band-songbook-lambda"
IMAGE_TAG="latest"

ECR_URI="${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO}"

echo "=== Creating ECR repository (if not exists) ==="
aws ecr describe-repositories --repository-names ${ECR_REPO} 2>/dev/null || \
    aws ecr create-repository --repository-name ${ECR_REPO}

echo "=== Logging in to ECR ==="
aws ecr get-login-password --region ${AWS_REGION} | \
    docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com

echo "=== Building Docker image ==="
docker build --platform linux/amd64 -t ${ECR_REPO}:${IMAGE_TAG} .

echo "=== Tagging image ==="
docker tag ${ECR_REPO}:${IMAGE_TAG} ${ECR_URI}:${IMAGE_TAG}

echo "=== Pushing image to ECR ==="
docker push ${ECR_URI}:${IMAGE_TAG}

echo "=== Updating Lambda function ==="
# Check if function exists
if aws lambda get-function --function-name ${FUNCTION_NAME} 2>/dev/null; then
    echo "Updating existing function..."
    aws lambda update-function-code \
        --function-name ${FUNCTION_NAME} \
        --image-uri ${ECR_URI}:${IMAGE_TAG}
else
    echo "Creating new function..."
    aws lambda create-function \
        --function-name ${FUNCTION_NAME} \
        --package-type Image \
        --code ImageUri=${ECR_URI}:${IMAGE_TAG} \
        --role arn:aws:iam::${AWS_ACCOUNT_ID}:role/cargo-lambda-role-71626fb1-b25c-4726-88fc-cf2f0162918c \
        --timeout 900 \
        --memory-size 2048 \
        --environment "Variables={RUST_LOG=info,SRCDIR=s3://zik-laurent/songs,SANDBOX=s3://zik-laurent/sandbox,SETTINGS=s3://zik-laurent/songs/settings.yml}"
fi

echo "=== Waiting for function to be ready ==="
aws lambda wait function-updated --function-name ${FUNCTION_NAME} 2>/dev/null || \
    aws lambda wait function-active --function-name ${FUNCTION_NAME}

echo "=== Updating function configuration ==="
aws lambda update-function-configuration \
    --function-name ${FUNCTION_NAME} \
    --timeout 900 \
    --memory-size 2048 \
    --environment "Variables={RUST_LOG=info,SRCDIR=s3://zik-laurent/songs,SANDBOX=s3://zik-laurent/sandbox,SETTINGS=s3://zik-laurent/songs/settings.yml}"

echo "=== Done! ==="
echo "Lambda function '${FUNCTION_NAME}' updated with Docker image."
echo "Image: ${ECR_URI}:${IMAGE_TAG}"
