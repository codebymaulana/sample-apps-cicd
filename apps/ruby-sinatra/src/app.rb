require "sinatra"
require "json"

set :bind, "0.0.0.0"
set :port, 8080

get "/" do
  "Hello from Ruby!"
end

get "/health" do
  content_type :json
  { status: "ok" }.to_json
end
