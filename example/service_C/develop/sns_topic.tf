resource "aws_sns_topic" "ServiceC_Event" {
  name         = "ServiceC"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceC_Event"
    }
  ]
}
POLICY
}
