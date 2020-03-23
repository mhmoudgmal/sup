require 'json'

def handler(event:, context:)
  { vars: ENV.to_h }
end
