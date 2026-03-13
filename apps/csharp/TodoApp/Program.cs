using TodoApp.Data;

var builder = WebApplication.CreateBuilder(args);

var port = Environment.GetEnvironmentVariable("PORT") ?? "5002";
builder.WebHost.UseUrls($"http://localhost:{port}");

// Add services
builder.Services.AddRazorPages();
builder.Services.AddSingleton(new TodoDb("Data Source=todos.db"));

var app = builder.Build();

// Auto-create tables and seed data
var db = app.Services.GetRequiredService<TodoDb>();
db.EnsureCreated();

if (!db.HasAnyLists())
{
    db.InsertList("Personal");
    db.InsertList("Work");

    // Get the lists we just created to grab their IDs
    var lists = db.GetAllLists();
    var personal = lists.First(l => l.Name == "Personal");
    var work = lists.First(l => l.Name == "Work");

    db.InsertTodo("Buy groceries", personal.Id);
    db.InsertTodo("Walk the dog", personal.Id);
    db.InsertTodo("Read a book", personal.Id);
    db.InsertTodo("Finish quarterly report", work.Id);
    db.InsertTodo("Review pull requests", work.Id);
    db.InsertTodo("Update documentation", work.Id);
}

// Configure pipeline
if (!app.Environment.IsDevelopment())
{
    app.UseExceptionHandler("/Error");
}

app.UseRouting();
app.UseAuthorization();
app.MapStaticAssets();
app.MapRazorPages()
   .WithStaticAssets();

app.Run();
