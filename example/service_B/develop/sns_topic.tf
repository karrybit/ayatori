resource "aws_sns_topic" "ServiceB_Event1" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceB_Event1"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceB_Event2" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceB_Event2"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceB_Event3" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceB_Event3"
    }
  ]
}
POLICY
}
