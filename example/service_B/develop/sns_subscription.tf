resource "aws_sns_topic_subscription" "ServiceC_Event" {
  topic_arn = "ServiceC_Event"
  protocol  = "sqs"
  endpoint  = ServiceB
}

resource "aws_sns_topic_subscription" "ServiceE_Event" {
  topic_arn = "ServiceE_Event"
  protocol  = "sqs"
  endpoint  = ServiceB
}
