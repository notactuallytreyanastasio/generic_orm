namespace TodoApp.Models;

public class TodoItem
{
    public int Id { get; set; }
    public string Title { get; set; } = string.Empty;
    public bool Completed { get; set; }
    public int ListId { get; set; }
    public string CreatedAt { get; set; } = string.Empty;
}
