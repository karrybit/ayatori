resource "aws_sns_topic" "ServiceD_Event" {
  name         = "ServiceD"
  display_name = ""
  policy       = <<POLICY
{
  "Statement": [
    {
      "Resource": "ServiceD_Event"
    }
  ]
}
POLICY
}
