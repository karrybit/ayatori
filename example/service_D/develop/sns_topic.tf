resource "aws_sns_topic" "ServiceD_Event1" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceD_Event1"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceD_Event2" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceD_Event2"
    }
  ]
}
POLICY
}

resource "aws_sns_topic" "ServiceD_Event3" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceD_Event3"
    }
  ]
}
POLICY
}
