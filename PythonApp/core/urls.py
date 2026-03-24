from django.urls import path
from api import views
from utils.bubble_sort import bubble_sort_view

urlpatterns = [
    path('', views.index),                               # Route: /
    path('users', views.user_list_create),               # Route: /users
    path('users/<int:user_id>', views.user_detail),      # Route: /users/:id
    path('django/sort', bubble_sort_view),               # Route: /django/sort
]