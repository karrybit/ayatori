resource "aws_sns_topic_subscription" "ServiceB_Event" {
  topic_arn = "ServiceB_Event"
  protocol  = "sqs"
  endpoint  = ServiceE
}

resource "aws_sns_topic_subscription" "ServiceD_Event" {
  topic_arn = "ServiceD_Event"
  protocol  = "sqs"
  endpoint  = ServiceE
}

resource "aws_sns_topic_subscription" "ServiceF_Event" {
  topic_arn = "ServiceF_Event"
  protocol  = "sqs"
  endpoint  = ServiceE
}
