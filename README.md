# 🌟 Pinfetch | _Информация о системе в стиле_ 🌈

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/yourusername/system-info/pulls)

Утилита для мгновенного получения красочного отчёта о вашей системе! Поддерживает 5 языков и автоматически определяет локализацию. 🖥️✨

<div align="center">
  <img src="https://media4.giphy.com/media/v1.Y2lkPTc5MGI3NjExazF3aW56endnMzRvd2U0MHVxbXMxamRpd3BvZGdmbndlMG56bzlrOCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/jBOOXxSJfG8kqMxT11/giphy.gif" width="400">
</div>

## 🚀 Особенности

- 🌍 **Мультиязычная поддержка** (Английский, Русский, Немецкий, Французский, Испанский)
- 🎨 **Цветной вывод** с подсветкой ключевых параметров
- 📊 Комплексная информация:
  - Пользователь и хостнейм
  - Версия ОС и ядра
  - Архитектура процессора
  - Нагрузка CPU и память
  - Разрешение экрана
  - Дисковое пространство
  - Время работы системы
- 🛠️ **Автоматическое определение локализации** через переменную `LANG`
- ✅ Совместимость с Linux-системами

## 📦 Установка

```bash
# Клонировать репозиторий
git clone https://github.com/SublimateTheBerry/pinfetch.git
cd pinfetch

# Собрать проект (требуется Rust 1.60+)
cargo build --release

# Запустить
./target/release/pinfetch

# Копировать в /bin для удобства
sudo cp /target/release/pinfetch /bin/pinfetch
```

## 🖥️ Пример вывода

```bash
user@awesome-pc
ОС: Ubuntu 22.04.3 LTS
Ядро: 5.15.0-78-generic
Архитектура: x86_64
Ядра CPU: 8
Загрузка CPU: 12.34%
Память: 2456 MB / 16048 MB (15.30%)
Разрешение: 1920x1080
Дисковое пространство: 45G / 120G
Время работы: 2 hours 30 minutes
```

## 🌐 Поддерживаемые языки

| Переменная LANG | Язык       | Пример меток          |
|-----------------|------------|-----------------------|
| `ru_RU`         | Русский    | Память, Ядра CPU      |
| `de_DE`         | Немецкий   | CPU-Auslastung        |
| `fr_FR`         | Французский| Cœurs CPU             |
| `es_ES`         | Испанский  | Tiempo de actividad   |
| Другие          | Английский | CPU Load, Uptime      |

Для принудительного изменения языка:
```bash
LANG=es_ES ./target/release/system-info
```

## 🛠️ Зависимости

- [os_info](https://crates.io/crates/os_info) - Информация об ОС
- [sys_info](https://crates.io/crates/sys_info) - Статистика системы
- [whoami](https://crates.io/crates/whoami) - Данные пользователя
- [colored](https://crates.io/crates/colored) - Цветной вывод

## 🤝 Как помочь проекту

1. Форкните репозиторий
2. Создайте ветку для фичи (`git checkout -b feature/AmazingFeature`)
3. Сделайте коммит изменений (`git commit -m 'Add AmazingFeature'`)
4. Запушьте ветку (`git push origin feature/AmazingFeature`)
5. Откройте Pull Request

## 📜 Лицензия

MIT © SublimateTheBerry - Смотрите файл [LICENSE](LICENSE) для деталей

---

**Протестировано с ❤️ на Linux-системах**  
_Вдохновлено neofetch и screenfetch_ 🖼️
