resource "aws_sns_topic_subscription" "ServiceA_Event" {
  topic_arn = "ServiceA_Event"
  protocol  = "sqs"
  endpoint  = ServiceG
}

resource "aws_sns_topic_subscription" "ServiceB_Event" {
  topic_arn = "ServiceB_Event"
  protocol  = "sqs"
  endpoint  = ServiceG
}
