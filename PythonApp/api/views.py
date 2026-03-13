import json
from django.shortcuts import render, get_object_or_404
from django.http import JsonResponse, HttpResponse
from django.views.decorators.csrf import csrf_exempt
from .models import User  # This matches your User struct in models.rs

def index(request):
    return render(request, 'index.html')

@csrf_exempt # Added because your Rust app didn't require CSRF tokens for POST/PUT
def user_list_create(request):
    if request.method == 'POST':
        data = json.loads(request.body)
        User.objects.create(name=data['name'], email=data['email'])
        return HttpResponse(status=201) # 201 Created
    return HttpResponse(status=405)

@csrf_exempt
def user_detail(request, user_id):
    user = get_object_or_404(User, id=user_id)
    
    if request.method == 'GET':
        return JsonResponse({"id": user.id, "name": user.name, "email": user.email})
    
    elif request.method == 'PUT':
        data = json.loads(request.body)
        user.email = data['email']
        user.save()
        return HttpResponse(status=200)
    
    elif request.method == 'DELETE':
        user.delete()
        return HttpResponse(status=204) # 204 No Content
        
    return HttpResponse(status=405)