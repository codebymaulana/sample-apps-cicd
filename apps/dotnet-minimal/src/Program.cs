var builder = WebApplication.CreateBuilder(args);
builder.WebHost.UseUrls("http://0.0.0.0:8080");

var app = builder.Build();

app.MapGet("/", () => "Hello from .NET!");

app.MapGet("/health", () => Results.Ok(new { status = "ok" }));

app.Run();
