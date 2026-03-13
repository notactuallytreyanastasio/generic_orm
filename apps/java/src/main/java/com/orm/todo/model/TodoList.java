package com.orm.todo.model;

import java.time.LocalDateTime;

/**
 * Plain POJO for a todo list. No JPA annotations -- mapped via JDBC + ORM.
 */
public class TodoList {

    private Long id;
    private String name;
    private LocalDateTime createdAt;

    // Derived counts populated by the repository layer
    private long completedCount;
    private long totalCount;

    public TodoList() {
    }

    public TodoList(String name) {
        this.name = name;
        this.createdAt = LocalDateTime.now();
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(LocalDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public long getCompletedCount() {
        return completedCount;
    }

    public void setCompletedCount(long completedCount) {
        this.completedCount = completedCount;
    }

    public long getTotalCount() {
        return totalCount;
    }

    public void setTotalCount(long totalCount) {
        this.totalCount = totalCount;
    }
}
