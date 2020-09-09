resource "aws_sns_topic_subscription" "ServiceA_Event" {
  topic_arn = "ServiceA_Event"
  protocol  = "sqs"
  endpoint  = ServiceF
}

resource "aws_sns_topic_subscription" "ServiceD_Event" {
  topic_arn = "ServiceD_Event"
  protocol  = "sqs"
  endpoint  = ServiceF
}
