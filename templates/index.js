export default [
    {% for recipe in recipes %}{
        "name": "{{recipe.name}}",
        "path": "{{recipe.path}}"
    }{% if loop.index < recipes.len() %},
    {% endif %}{% endfor %}
]