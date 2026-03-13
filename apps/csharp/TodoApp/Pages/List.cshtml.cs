using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using TodoApp.Data;
using TodoApp.Models;

namespace TodoApp.Pages;

public class ListModel : PageModel
{
    private readonly TodoDb _db;

    public ListModel(TodoDb db)
    {
        _db = db;
    }

    public TodoList? TodoList { get; set; }
    public int? EditingTodoId { get; set; }

    public void OnGet(int id, int? editTodoId)
    {
        TodoList = _db.GetList(id);
        EditingTodoId = editTodoId;
    }

    public IActionResult OnPostAddTodo(int id, string title)
    {
        if (string.IsNullOrWhiteSpace(title))
            return RedirectToPage(new { id });

        // Verify list exists
        var list = _db.GetList(id);
        if (list == null)
            return RedirectToPage("/Index");

        _db.InsertTodo(title.Trim(), id);
        return RedirectToPage(new { id });
    }

    public IActionResult OnPostToggleTodo(int id, int todoId)
    {
        _db.ToggleTodo(todoId);
        return RedirectToPage(new { id });
    }

    public IActionResult OnPostUpdateTodo(int id, int todoId, string title)
    {
        if (!string.IsNullOrWhiteSpace(title))
        {
            _db.UpdateTodo(todoId, title.Trim());
        }
        return RedirectToPage(new { id });
    }

    public IActionResult OnPostDeleteTodo(int id, int todoId)
    {
        _db.DeleteTodo(todoId);
        return RedirectToPage(new { id });
    }
}
