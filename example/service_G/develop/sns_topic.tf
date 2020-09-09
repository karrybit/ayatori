resource "aws_sns_topic" "ServiceG_Event" {
  name         = "ServiceF"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceG_Event"
    }
  ]
}
POLICY
}
