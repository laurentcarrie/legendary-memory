#!/bin/bash
set -e

# Configuration
FUNCTION_NAME="band-songbook"
AWS_REGION="eu-west-3"
S3_BUCKET="zik-laurent"
S3_PREFIX="songs/"

# Environment variables for the Lambda
SRCDIR="s3://zik-laurent/songs"
SANDBOX="s3://zik-laurent/sandbox"
SETTINGS="s3://zik-laurent/songs/settings.yml"

echo "=== Building Lambda function ==="
cargo lambda build --release --bin band-songbook-lambda

echo "=== Deploying Lambda function ==="
cargo lambda deploy $FUNCTION_NAME \
    --timeout 900 \
    --memory 1024 \
    --env-var RUST_LOG=info \
    --env-var SRCDIR="$SRCDIR" \
    --env-var SANDBOX="$SANDBOX" \
    --env-var SETTINGS="$SETTINGS"

echo "=== Getting Lambda ARN ==="
LAMBDA_ARN=$(aws lambda get-function --function-name $FUNCTION_NAME --query 'Configuration.FunctionArn' --output text)
echo "Lambda ARN: $LAMBDA_ARN"

echo "=== Adding S3 permission to Lambda ==="
# Remove existing permission if it exists (ignore errors)
aws lambda remove-permission \
    --function-name $FUNCTION_NAME \
    --statement-id s3-trigger-permission 2>/dev/null || true

# Add permission for S3 to invoke Lambda
aws lambda add-permission \
    --function-name $FUNCTION_NAME \
    --statement-id s3-trigger-permission \
    --action lambda:InvokeFunction \
    --principal s3.amazonaws.com \
    --source-arn "arn:aws:s3:::$S3_BUCKET" \
    --source-account $(aws sts get-caller-identity --query Account --output text)

echo "=== Configuring S3 bucket notification ==="
# Create notification configuration
cat > /tmp/s3-notification.json << EOF
{
    "LambdaFunctionConfigurations": [
        {
            "Id": "band-songbook-trigger",
            "LambdaFunctionArn": "$LAMBDA_ARN",
            "Events": [
                "s3:ObjectCreated:*",
                "s3:ObjectRemoved:*"
            ],
            "Filter": {
                "Key": {
                    "FilterRules": [
                        {
                            "Name": "prefix",
                            "Value": "$S3_PREFIX"
                        }
                    ]
                }
            }
        }
    ]
}
EOF

aws s3api put-bucket-notification-configuration \
    --bucket $S3_BUCKET \
    --notification-configuration file:///tmp/s3-notification.json

echo "=== Done! ==="
echo "Lambda function '$FUNCTION_NAME' is now triggered when files in s3://$S3_BUCKET/$S3_PREFIX are modified."
echo ""
echo "Configuration:"
echo "  SRCDIR: $SRCDIR"
echo "  SANDBOX: $SANDBOX"
echo "  SETTINGS: $SETTINGS"
