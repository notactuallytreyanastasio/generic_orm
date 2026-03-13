namespace TodoApp.Models;

public class TodoList
{
    public int Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public string CreatedAt { get; set; } = string.Empty;
    public List<TodoItem> Todos { get; set; } = new();
}
