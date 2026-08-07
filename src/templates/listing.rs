/// Get the HTML content to render the home/listing page.
///
/// # See Also
///
/// - This page is served as a response for the `/home` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<!--suppress JSUnresolvedLibraryURL -->
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>RuStream - Self-hosted Streaming Engine - v{{ version }}</title>
    <meta property="og:type" content="MediaStreaming">
    <meta name="keywords" content="Rust, streaming, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.css">
    <!-- CSS and JS for night mode -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/jquery/2.2.2/jquery.min.js"></script>
    <script type="text/javascript" src="https://thevickypedia.github.io/open-source/nightmode/night.js" defer></script>
    <link rel="stylesheet" type="text/css" href="https://thevickypedia.github.io/open-source/nightmode/night.css">
    <!-- Navbar CSS -->
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        body {
            margin-left: 1%;  /* 1% away from left corner */
            padding: 0.5%  /* 0.5% away from any surrounding elements */
        }
        small {
            font-size: 16px;
        }
        .navbar {
            position: relative;
            display: flex;
            align-items: center;
            gap: 10px;
            margin: 0.5% 0 1.5%;
        }
        .navbar button {
            border: none;
            padding: 8px 14px;
            font-size: 15px;
            cursor: pointer;
        }
    </style>
    <style>
        .dropbtn {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            font-size: 16px;
            border: none;
            cursor: pointer;
        }
        .dropdown {
            position: absolute;
            top: 3.8%;
            right: 30px;
            padding: 10px 24px;
            display: inline-block;
        }
        .dropdown-content {
            display: none;
            position: absolute;
            top: 40px;  /* Distance from the user icon button */
            right: 30px;
            width: 160px;
            min-width: auto;
            box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);  /* Basically, black with 20% opacity */
            z-index: 1;
        }
        .dropdown-content a {
            padding: 12px 16px;
            text-decoration: none;
            display: block;
        }
        .dropdown:hover .dropdown-content {display: block;}
    </style>
    <!-- Title and listing CSS -->
    <style>
        a:hover, a:active { font-size: 102%; opacity: 0.5; }
        a:link { color: blue; }
        a:visited { color: blue; }
        ol {
            list-style: none;
            padding-left: 0;
        }
        li {
            margin: 0.6rem 0;
            list-style-type: none; /* Hide default marker */
        }
        .file-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: 12px;
            padding: 0.4rem 0.6rem;
        }
        .file-row:hover {
            background: rgba(65, 105, 225, 0.08);
        }
        .file-info {
            display: flex;
            align-items: center;
            gap: 6px;
            flex: 1;
            min-width: 0;
        }
        .file-info a {
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }
        .file-meta {
            color: #888888;
            font-size: 0.85em;
            white-space: nowrap;
        }
        .file-actions {
            display: flex;
            align-items: center;
            gap: 6px;
            flex-shrink: 0;
        }
        .file-actions button {
            border: 1px solid #cccccc;
            background: transparent;
            padding: 4px 10px;
            font-size: 13px;
            cursor: pointer;
            border-radius: 4px;
        }
    </style>
    <!-- Convert dialog and toast CSS -->
    <style>
        .modal-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.4);
            align-items: center;
            justify-content: center;
            z-index: 999;
        }
        .modal {
            background: #ffffff;
            border-radius: 8px;
            padding: 20px 24px;
            min-width: 320px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.25);
        }
        .modal h3 {
            margin: 0 0 8px;
        }
        .modal .file-name {
            color: #555555;
            margin: 0 0 12px;
            word-break: break-all;
        }
        .modal select {
            width: 100%;
            padding: 8px;
            margin: 8px 0 16px;
            font-size: 15px;
            border: 1px solid #cccccc;
            border-radius: 4px;
            box-sizing: border-box;
        }
        .modal-actions {
            display: flex;
            gap: 8px;
            justify-content: flex-end;
        }
        .modal-actions button {
            border: 1px solid #cccccc;
            background: transparent;
            padding: 6px 14px;
            font-size: 14px;
            cursor: pointer;
            border-radius: 4px;
        }
        .toast {
            display: none;
            position: fixed;
            bottom: 30px;
            left: 50%;
            transform: translateX(-50%);
            padding: 12px 20px;
            border-radius: 6px;
            color: #ffffff;
            font-size: 15px;
            max-width: 90%;
            word-break: break-word;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
            z-index: 1000;
        }
        .toast.success { background: #2e7d32; }
        .toast.error { background: #c62828; }
        .toast.info { background: #1565c0; }
    </style>
    <style>
        /* Style for context menu */
        .context-menu {
            position: absolute;
            background-color: #fff;
            border: 1px solid #ccc;
            padding: 5px 0;
            box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
        }
        .context-menu-item {
            padding: 5px 10px;
            cursor: pointer;
            background-color: #fff !important; /* White background */
            color: #000 !important; /* Black font */
        }
        .context-menu-item:hover {
            background-color: #000 !important; /* Black background */
            color: #fff !important; /* White font */
        }
        .icon {
            background-color: #fff !important; /* White background */
            color: #000 !important; /* Black font */
        }
        .icon:hover {
            background-color: #000 !important; /* Black background */
            color: #fff !important; /* White font */
        }
    </style>
</head>
<noscript>
    <style>
        body {
            width: 100%;
            height: 100%;
            overflow: hidden;
        }
    </style>
    <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515;">
        <h2 style="margin-top:5%">This page requires JavaScript
            to be enabled.
            <br><br>
            Please refer <a href="https://www.enable-javascript.com/">enable-javascript</a> for how to.
        </h2>
        <form>
            <button type="submit" onClick="<meta httpEquiv='refresh' content='0'>">RETRY</button>
        </form>
    </div>
</noscript>
<body translate="no">
    <div class="toggler fa fa-moon-o"></div>
    <div class="navbar">
        <button onclick="goHome()"><i class="fa fa-home"></i> Home</button>
        <button onclick="goBack()"><i class="fa fa-backward"></i> Back</button>
        <button onclick="upload()"><i class="fa-solid fa-cloud-arrow-up"></i> Upload</button>
    </div>
    <div class="dropdown">
        <button class="dropbtn"><i class="fa fa-user"></i></button>
        <div class="dropdown-content">
            <a onclick="goProfile()" style="cursor: pointer;"><i class="fa-solid fa-user-lock"></i> {{ user }}</a>
            <a onclick="logOut()" style="cursor: pointer"><i class="fa fa-sign-out"></i> logout</a>
        </div>
    </div>
    <!-- Convert dialog (hidden by default) -->
    <div id="convertDialog" class="modal-overlay">
        <div class="modal">
            <h3>Convert video</h3>
            <p class="file-name" id="convertFileName"></p>
            <label for="convertFormat">Convert to:</label>
            <select id="convertFormat">
                {% for format in video_formats %}
                    <option value="{{ format }}">{{ format }}</option>
                {% endfor %}
            </select>
            <div class="modal-actions">
                <button onclick="startConversion()"><i class="fa-solid fa-wand-magic-sparkles"></i>&nbsp;&nbsp;Convert</button>
                <button onclick="closeConvertDialog()">Cancel</button>
            </div>
        </div>
    </div>
    <!-- Toast notification (hidden by default) -->
    <div id="toast" class="toast"></div>
    <!-- Context menu template (hidden by default) -->
    <div id="contextMenu" class="context-menu icon" style="display: none;">
        <div class="context-menu-item" onclick="editItem(currentPath, 'delete')"><i class="fa-regular fa-trash-can"></i>&nbsp;&nbsp;Delete</div>
        <div class="context-menu-item" onclick="editItem(currentPath, 'rename')"><i class="fa-solid fa-pen"></i>&nbsp;&nbsp;Rename</div>
    </div>
    {% if custom_title %}
        <h1>{{ custom_title }}</h1>
    {% else %}
        <h1>Welcome to RuStream <small>v{{ version }}</small></h1>
    {% endif %}
    <hr>
    {% if dir_name or files or directories or secured_directories %}
        <!-- Display directory name if within subdir -->
        {% if dir_name %}
            <h3>{{ dir_name }}</h3>
        {% endif %}
        <!-- Display number of files and list the files -->
        {% if files %}
            <h3>Files {{ files|length }}</h3>
            <ol>
                {% for file in files %}
                    <li class="file-row">
                        <div class="file-info">
                            <i class="{{ file.font }}"></i>&nbsp;&nbsp;<a href="{{ file.path }}">{{ file.name }}</a>
                            {% if file.size or file.duration %}
                                <span class="file-meta">
                                    {% if file.size %}{{ file.size }}{% endif %}
                                    {% if file.duration %} &middot; {{ file.duration }}{% endif %}
                                </span>
                            {% endif %}
                        </div>
                        <div class="file-actions">
                            {% if ffmpeg_enabled and file.video == 'true' %}
                                <button onclick="openConvertDialog('{{ file.path }}')" title="Convert format"><i class="fa-solid fa-wand-magic-sparkles"></i>&nbsp;&nbsp;Convert</button>
                            {% endif %}
                            <button onclick="downloadFile('{{ file.path }}')" title="Download"><i class="fa-solid fa-download"></i></button>
                            <button onclick="editItem('{{ file.path }}', 'rename')" title="Rename"><i class="fa-solid fa-pen"></i></button>
                            <button onclick="editItem('{{ file.path }}', 'delete')" title="Delete"><i class="fa-regular fa-trash-can"></i></button>
                        </div>
                    </li>
                {% endfor %}
            </ol>
        {% endif %}
        <!-- Display number of directories and list the directories -->
        {% if directories %}
            <h3>Directories {{ directories|length }}</h3>
            {% for directory in directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
        {% if secured_directories %}
            <h3>Secured Directory</h3>
            {% for directory in secured_directories %}
                <li><i class="{{ directory.font }}"></i>&nbsp;&nbsp;<a oncontextmenu="showContextMenu(event, '{{ directory.path }}', true)" href="{{ directory.path }}">{{ directory.name }}</a></li>
            {% endfor %}
        {% endif %}
    {% else %}
        <h3 style="text-align: center">No content was rendered by the server</h3>
    {% endif %}
    <hr>
    <script>
        const BASE_URL = "{{ base_url }}";
        function prefixed(path) { return BASE_URL === "/" ? path : BASE_URL + path; }
        function goHome() {
            window.location.href = prefixed("/home");
        }
        function goProfile() {
            window.location.href = prefixed('/profile');
        }
        function logOut() {
            window.location.href = prefixed("/logout");
        }
        function upload() {
            window.location.href = prefixed("/upload");
        }
        function goBack() {
            window.history.back();
        }
    </script>
    <script>
        function downloadFile(path) {
            let filePath = path.replace(/^stream\//, '');
            window.location.href = window.location.origin + prefixed('/download?file=') + encodeURIComponent(filePath);
        }

        const VIDEO_FORMATS = [{% for format in video_formats %}'{{ format }}',{% endfor %}];

        let convertTargetPath = null;
        let convertTimer = null;

        function showToast(message, type) {
            let toast = document.getElementById('toast');
            toast.textContent = message;
            toast.className = 'toast ' + (type || 'info');
            toast.style.display = 'block';
            clearTimeout(convertTimer);
            convertTimer = setTimeout(function() {
                toast.style.display = 'none';
            }, 5000);
        }

        function openConvertDialog(path) {
            let fileName = extractFileName(path);
            let currentFormat = fileName.split('.').pop().toLowerCase();
            let select = document.getElementById('convertFormat');
            select.innerHTML = '';
            VIDEO_FORMATS.forEach(function(format) {
                if (format === currentFormat) {
                    return;
                }
                let option = document.createElement('option');
                option.value = format;
                option.text = format;
                select.appendChild(option);
            });
            if (select.options.length === 0) {
                showToast(`No other format is available for '${currentFormat}'`, 'error');
                return;
            }
            convertTargetPath = path;
            document.getElementById('convertFileName').innerText = fileName;
            document.getElementById('convertDialog').style.display = 'flex';
        }

        function closeConvertDialog() {
            document.getElementById('convertDialog').style.display = 'none';
        }

        function startConversion() {
            let format = document.getElementById('convertFormat').value;
            if (!format || convertTargetPath === null) {
                return;
            }
            closeConvertDialog();
            let fileName = extractFileName(convertTargetPath);
            let trueURL = window.location.href + '/' + fileName;
            let http = new XMLHttpRequest();
            http.open('POST', window.location.origin + prefixed('/convert'), true);
            http.setRequestHeader('Content-Type', 'application/json');
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 202) {
                        let response = JSON.parse(http.responseText);
                        showToast(`Converting '${fileName}' to ${format}...`, 'info');
                        pollConversion(response.job, fileName, format);
                    } else {
                        if (http.responseText !== "") {
                            showToast(`Error: ${http.responseText}`, 'error');
                        } else {
                            showToast(`Error: ${http.statusText}`, 'error');
                        }
                    }
                }
            };
            http.send(JSON.stringify({
                url_locator: trueURL,
                path_locator: convertTargetPath,
                new_format: format
            }));
        }

        function pollConversion(jobId, fileName, format) {
            let poll = setInterval(function() {
                let status = new XMLHttpRequest();
                status.open('GET', window.location.origin + prefixed('/convert/status/') + jobId, true);
                status.onreadystatechange = function() {
                    if (status.readyState !== XMLHttpRequest.DONE) {
                        return;
                    }
                    if (status.status === 200) {
                        let job = JSON.parse(status.responseText);
                        if (job.state === 'running') {
                            return;
                        }
                        clearInterval(poll);
                        if (job.state === 'done') {
                            showToast(`'${fileName}' was converted to ${format} successfully`, 'success');
                            setTimeout(function() {
                                window.location.reload();
                            }, 2000);
                        } else {
                            showToast(`Conversion failed: ${job.detail}`, 'error');
                        }
                    } else {
                        clearInterval(poll);
                        showToast('Failed to fetch conversion status', 'error');
                    }
                };
                status.send();
            }, 1500);
        }
    </script>
    <script>
        var contextMenu = document.getElementById('contextMenu');

        // Function to show context menu
        function showContextMenu(event, path, isDir = false) {
            event.preventDefault();

            // Set the global variable to the current file path
            currentPath = path;
            directory = isDir;

            // Calculate the appropriate coordinates for the context menu
            var mouseX = event.clientX;
            var mouseY = event.clientY;
            var windowWidth = window.innerWidth;
            var windowHeight = window.innerHeight;
            var contextMenuWidth = contextMenu.offsetWidth;
            var contextMenuHeight = contextMenu.offsetHeight;
            var scrollX = window.scrollX || window.pageXOffset;
            var scrollY = window.scrollY || window.pageYOffset;

            // Adjust the coordinates considering the scroll position and moving 2 pixels away from the mouse pointer
            var menuX = mouseX + scrollX + contextMenuWidth > windowWidth ? mouseX + scrollX - contextMenuWidth - 2 : mouseX + scrollX + 2;
            var menuY = mouseY + scrollY + contextMenuHeight > windowHeight ? mouseY + scrollY - contextMenuHeight - 2 : mouseY + scrollY + 2;

            // Position the context menu at the calculated coordinates
            contextMenu.style.left = menuX + 'px';
            contextMenu.style.top = menuY + 'px';
            contextMenu.style.display = 'block';
        }

        function editAction(action, trueURL, relativePath, newName) {
            let http = new XMLHttpRequest();
            http.open('POST', window.location.origin + prefixed('/edit'), true);  // asynchronous session
            http.setRequestHeader('Content-Type', 'application/json'); // Set content type to JSON
            http.setRequestHeader('edit-action', action);
            http.onreadystatechange = function() {
                if (http.readyState === XMLHttpRequest.DONE) {
                    if (http.status === 200) {
                        window.location.reload();
                    } else {
                        if (http.responseText !== "") {
                            alert(`Error: ${http.responseText}`);
                        } else {
                            alert(`Error: ${http.statusText}`);
                        }
                    }
                }
            };
            let data = {
                url_locator: trueURL,
                path_locator: relativePath,
                new_name: newName
            };
            http.send(JSON.stringify(data));
        }

        function getConfirmation(fileName, action) {
            let confirmation = confirm(`Are you sure you want to ${action}?\n\n'${fileName}'`);
            if (!confirmation) {
                contextMenu.style.display = 'none';
                return false;
            }
            return true;
        }

        function extractFileName(path) {
            // Find the last occurrence of either '/' or '\'
            const lastIndex = Math.max(path.lastIndexOf('/'), path.lastIndexOf('\\'));

            // Extract the filename using substring
            return path.substring(lastIndex + 1);
        }

        function isValidName(oldName, newName) {
            // Condition 1 - Validate if the new filename is the same as old.
            if (oldName === newName) {
                alert(`New name is the same as old\n\n'${oldName}'=='${newName}'`);
            }
            // Condition 2 - Validate if the new filename starts or ends with . or _
            if (newName.startsWith('_') || newName.endsWith('_') ||
                newName.startsWith('.') || newName.endsWith('.')) {
                alert(`New name cannot start or end with '.' or '_'\n\n${newName}`);
                return false;
            }
            // Condition 3 - Validate if the new filename and the old has the same file extension.
            const oldExtension = oldName.split('.').pop();
            const newExtension = newName.split('.').pop();
            // Check condition 3
            if (oldExtension !== newExtension) {
                alert(`File extension cannot be changed\n\n'${newExtension}' => '${oldExtension}'`);
                return false;
            }
            // Condition 4 - Validate if the new filename has at least one character, apart from the file extension.
            if (newName.length <= oldExtension.length + 1) {
                alert(`At least one character is required as filename\n\nReceived ${newName.length}`);
                return false;
            }
            return true;
        }

        // Function to handle delete/rename action
        function editItem(relativePath, action) {
            contextMenu.style.display = 'none';

            let fileName = extractFileName(relativePath);
            if (action === 'delete') {
                let pass = getConfirmation(fileName, action);
                if (!pass) {
                    return;
                }
                var newName = null;
            } else {
                if (directory) {
                    alert("Only a 'delete' action is permitted on directories");
                    return;
                }
                var newName = prompt(`Enter a new name for the file\n\nCurrent: ${fileName}\n`);
                if (!isValidName(fileName, newName)) {
                    return;
                }
            }
            let trueURL = window.location.href + '/' + fileName;
            editAction(action, trueURL, relativePath, newName);
        }

        // Hide context menu when clicking outside
        document.addEventListener('click', function(event) {
            if (event.target !== contextMenu && !contextMenu.contains(event.target)) {
                contextMenu.style.display = 'none';
            }
        });
        </script>
</body>
</html>
"###.to_string()
}
