resource "aws_sns_topic_subscription" "ServiceA_Event3" {
  topic_arn = "ServiceA_Event3"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceB_Event3" {
  topic_arn = "ServiceB_Event1"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceD_Event3" {
  topic_arn = "ServiceD_Event3"
  protocol  = "sqs"
  endpoint  = ServiceA
}
