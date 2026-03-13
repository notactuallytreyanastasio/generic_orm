package com.orm.todo;

import com.orm.todo.repository.TodoRepository;
import org.springframework.boot.CommandLineRunner;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Component;

@Component
public class DataLoader implements CommandLineRunner {

    private final TodoRepository repo;
    private final JdbcTemplate jdbc;

    public DataLoader(TodoRepository repo, JdbcTemplate jdbc) {
        this.repo = repo;
        this.jdbc = jdbc;
    }

    @Override
    public void run(String... args) {
        if (repo.countLists() > 0) {
            return;
        }

        // Seed list 1: Work Tasks
        repo.insertList("Work Tasks");
        Long workId = jdbc.queryForObject(
            "SELECT id FROM lists WHERE name = 'Work Tasks' ORDER BY id DESC LIMIT 1", Long.class);

        repo.insertItem("Review pull requests", workId);
        repo.insertItem("Update project documentation", workId);
        repo.insertItem("Fix login page bug", workId);
        repo.insertItem("Set up CI pipeline", workId, true);

        // Seed list 2: Groceries
        repo.insertList("Groceries");
        Long groceriesId = jdbc.queryForObject(
            "SELECT id FROM lists WHERE name = 'Groceries' ORDER BY id DESC LIMIT 1", Long.class);

        repo.insertItem("Milk", groceriesId);
        repo.insertItem("Eggs", groceriesId);
        repo.insertItem("Bread", groceriesId);
        repo.insertItem("Coffee beans", groceriesId, true);
        repo.insertItem("Butter", groceriesId, true);
    }
}
