package com.orm.todo.model;

import java.time.LocalDateTime;

/**
 * Plain POJO for a todo item. No JPA annotations -- mapped via JDBC + ORM.
 */
public class TodoItem {

    private Long id;
    private String title;
    private Boolean completed;
    private LocalDateTime createdAt;
    private Long listId;

    public TodoItem() {
    }

    public TodoItem(String title, Long listId) {
        this.title = title;
        this.listId = listId;
        this.completed = false;
        this.createdAt = LocalDateTime.now();
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public Boolean getCompleted() {
        return completed;
    }

    public void setCompleted(Boolean completed) {
        this.completed = completed;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(LocalDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public Long getListId() {
        return listId;
    }

    public void setListId(Long listId) {
        this.listId = listId;
    }
}
