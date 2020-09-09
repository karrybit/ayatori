resource "aws_sns_topic_subscription" "ServiceA_Event" {
  topic_arn = "ServiceA_Event"
  protocol  = "sqs"
  endpoint  = ServiceD
}

resource "aws_sns_topic_subscription" "ServiceB_Event" {
  topic_arn = "ServiceB_Event"
  protocol  = "sqs"
  endpoint  = ServiceD
}

resource "aws_sns_topic_subscription" "ServiceE_Event" {
  topic_arn = "ServiceE_Event"
  protocol  = "sqs"
  endpoint  = ServiceD
}

resource "aws_sns_topic_subscription" "ServiceG_Event" {
  topic_arn = "ServiceG_Event"
  protocol  = "sqs"
  endpoint  = ServiceD
}
