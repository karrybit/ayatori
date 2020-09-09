resource "aws_sns_topic_subscription" "ServiceC_Event" {
  topic_arn = "ServiceC_Event"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceD_Event" {
  topic_arn = "ServiceD_Event"
  protocol  = "sqs"
  endpoint  = ServiceA
}
