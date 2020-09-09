resource "aws_sns_topic_subscription" "ServiceB_Event1" {
  topic_arn = "ServiceB_Event1"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceC_Event1" {
  topic_arn = "ServiceC_Event1"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceD_Event1" {
  topic_arn = "ServiceD_Event1"
  protocol  = "sqs"
  endpoint  = ServiceA
}
