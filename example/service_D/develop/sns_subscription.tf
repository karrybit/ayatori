resource "aws_sns_topic_subscription" "ServiceA_Event1" {
  topic_arn = "ServiceA_Event1"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceB_Event2" {
  topic_arn = "ServiceB_Event2"
  protocol  = "sqs"
  endpoint  = ServiceA
}

resource "aws_sns_topic_subscription" "ServiceC_Event3" {
  topic_arn = "ServiceC_Event3"
  protocol  = "sqs"
  endpoint  = ServiceA
}
