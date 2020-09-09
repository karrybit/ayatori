resource "aws_sns_topic_subscription" "ServiceA_Event2" {
  topic_arn = "ServiceA_Event2"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceC_Event2" {
  topic_arn = "ServiceC_Event2"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceD_Event2" {
  topic_arn = "ServiceD_Event2"
  protocol  = "sqs"
  endpoint  = ServiceA
}
