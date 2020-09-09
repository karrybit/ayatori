resource "aws_sns_topic" "ServiceB_Event" {
  name         = "ServiceB"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceB_Event"
    }
  ]
}
POLICY
}
