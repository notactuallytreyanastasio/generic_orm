using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using TodoApp.Data;
using TodoApp.Models;

namespace TodoApp.Pages;

public class IndexModel : PageModel
{
    private readonly TodoDb _db;

    public IndexModel(TodoDb db)
    {
        _db = db;
    }

    public List<TodoList> Lists { get; set; } = new();
    public int? EditingListId { get; set; }

    public void OnGet(int? editId)
    {
        Lists = _db.GetAllLists();
        EditingListId = editId;
    }

    public IActionResult OnPostCreate(string name)
    {
        if (string.IsNullOrWhiteSpace(name))
            return RedirectToPage();

        _db.InsertList(name.Trim());
        return RedirectToPage();
    }

    public IActionResult OnPostUpdate(int id, string name)
    {
        if (!string.IsNullOrWhiteSpace(name))
        {
            _db.UpdateList(id, name.Trim());
        }
        return RedirectToPage();
    }

    public IActionResult OnPostDelete(int id)
    {
        _db.DeleteList(id);
        return RedirectToPage();
    }
}
