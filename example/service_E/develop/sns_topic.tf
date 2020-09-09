resource "aws_sns_topic" "ServiceE_Event" {
  name         = "ServiceE"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceE_Event"
    }
  ]
}
POLICY
}
