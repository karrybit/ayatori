resource "aws_sns_topic" "ServiceA_Event" {
  name         = "ServiceA"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceA_Event"
    }
  ]
}
POLICY
}
