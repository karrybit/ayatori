resource "aws_sns_topic" "ServiceA_Event1" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceA_Event1"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceA_Event2" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceA_Event2"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceA_Event3" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceA_Event3"
    }
  ]
}
POLICY
}
