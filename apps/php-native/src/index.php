<?php

$path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);

if ($path === '/health') {
    header('Content-Type: application/json');
    echo json_encode(['status' => 'ok']);
} else {
    echo 'Hello from PHP!';
}
