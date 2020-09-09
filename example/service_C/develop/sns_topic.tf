resource "aws_sns_topic" "ServiceC_Event1" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceC_Event1"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceC_Event2" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceC_Event2"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceC_Event3" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceC_Event3"
    }
  ]
}
POLICY
}
