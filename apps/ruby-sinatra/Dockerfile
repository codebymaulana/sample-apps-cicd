FROM ruby:3.3-slim

WORKDIR /app

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser

COPY src/Gemfile .
RUN bundle install

COPY src/app.rb .

RUN chown -R appuser:appgroup /app
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD ruby -e "require 'net/http'; Net::HTTP.get(URI('http://localhost:8080/health'))"

CMD ["bundle", "exec", "ruby", "app.rb"]
