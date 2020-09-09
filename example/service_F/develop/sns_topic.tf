resource "aws_sns_topic" "ServiceF_Event" {
  name         = "ServiceF"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceF_Event"
    }
  ]
}
POLICY
}
