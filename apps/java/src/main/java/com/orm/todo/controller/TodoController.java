package com.orm.todo.controller;

import com.orm.todo.model.TodoItem;
import com.orm.todo.model.TodoList;
import com.orm.todo.repository.TodoRepository;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.*;

@Controller
public class TodoController {

    private final TodoRepository repo;

    public TodoController(TodoRepository repo) {
        this.repo = repo;
    }

    // --- Lists ---

    @GetMapping("/")
    public String index(Model model) {
        model.addAttribute("lists", repo.findAllLists());
        return "index";
    }

    @PostMapping("/lists")
    public String createList(@RequestParam String name) {
        if (name != null && !name.trim().isEmpty()) {
            repo.insertList(name.trim());
        }
        return "redirect:/";
    }

    @PostMapping("/lists/{id}/delete")
    public String deleteList(@PathVariable Long id) {
        repo.deleteList(id);
        return "redirect:/";
    }

    // --- Single List with Todos ---

    @GetMapping("/lists/{id}")
    public String showList(@PathVariable Long id, Model model) {
        TodoList list = repo.findListById(id);
        if (list == null) {
            return "redirect:/";
        }
        model.addAttribute("list", list);
        model.addAttribute("todos", repo.findItemsByListId(id));
        return "list";
    }

    @PostMapping("/lists/{id}/todos")
    public String addTodo(@PathVariable Long id, @RequestParam String title) {
        TodoList list = repo.findListById(id);
        if (list != null && title != null && !title.trim().isEmpty()) {
            repo.insertItem(title.trim(), id);
        }
        return "redirect:/lists/" + id;
    }

    // --- Todo Actions ---

    @PostMapping("/todos/{id}/toggle")
    public String toggleTodo(@PathVariable Long id) {
        TodoItem item = repo.findItemById(id);
        if (item != null) {
            repo.toggleItem(id);
            return "redirect:/lists/" + item.getListId();
        }
        return "redirect:/";
    }

    @PostMapping("/todos/{id}/delete")
    public String deleteTodo(@PathVariable Long id) {
        TodoItem item = repo.findItemById(id);
        if (item != null) {
            Long listId = item.getListId();
            repo.deleteItem(id);
            return "redirect:/lists/" + listId;
        }
        return "redirect:/";
    }

    @PostMapping("/todos/{id}/edit")
    public String editTodo(@PathVariable Long id, @RequestParam String title) {
        TodoItem item = repo.findItemById(id);
        if (item != null && title != null && !title.trim().isEmpty()) {
            repo.updateItemTitle(id, title.trim());
            return "redirect:/lists/" + item.getListId();
        }
        return "redirect:/";
    }
}
