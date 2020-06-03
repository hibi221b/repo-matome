pub const REPO_MATOME_INDEX_HTML: &'static str =r###"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>repo-matome</title>

    <style>
        body { 
            margin: 0px 0px 21px 0px;
            background-color: #15202b;
            color: #D5DBDB;
        }

        .container {
            padding: 16px;
            display: grid;
            grid-template-columns: 1fr minmax(460px, 3fr) 1fr;
        }

        .header {
            margin-bottom: 16px;
            grid-column-start: 2;
            grid-column-end: 3;

            grid-row-start: 1;
            grid-row-end: auto;
        }

        .main {
            grid-column-start: 2;
            grid-column-end: auto;

            grid-row-start: 2;
            grid-row-end: auto;
        }

        #jsonFile {
            display: none;
        }

        #selectFile {
            color: #1da1f2;
            cursor: pointer;
            padding: 8px 24px 8px 24px;
            display: inline-block;
            border-radius: 32px;
            border: 1px solid #1da1f2;
            margin-bottom: 4px;
        }

        #selectFile:hover {
            backdrop-filter: blur(8px);
            backdrop-filter: brightness(40%);
        }

        #selectedFileName {
            margin-top: 24px;
            margin-bottom: 0px;
            text-align: center;
        }
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1>repo-matome menu</h1>
            <label id="selectFile">
            select json file
                <input type="file" id="jsonFile" accept="application/json">
            </label>
            <h4 id="selectedFileName"></h4>
        </header>

        <main class="main" id="mainId"></main>
    </div>

    <script>
        if(!window.FileReader) {
            if(window.confirm("This browser does not support FileReader.")) {
                window.close();
            }
        }

        const main = document.getElementById("mainId");
        const input = document.getElementById("jsonFile");

        input.addEventListener("change", (e) => {
            const selectedJsonFile = e.target.value.substring(12);
            const jsonPathRegex = {
                number: "No-",
                lang: "-lang_",
                range: "-range_"
            };

            if(!selectedJsonFile.includes(jsonPathRegex.number) || !selectedJsonFile.includes(jsonPathRegex.lang) || !selectedJsonFile.includes(jsonPathRegex.range)) {
                //unexpected json file 
                window.alert("select $HOME/Downloads/repo-matome-result-dir/XXXXX.json");

            } else {

                const pTagJsonFile = document.getElementById("selectedFileName");
                pTagJsonFile.textContent = selectedJsonFile;

                //- if the contents of json are already visible on index.html, delete them.
                if(document.getElementById("topDivId") != null) {
                    main.removeChild(document.getElementById("topDivId"));
                }

                const fileReader = new FileReader();
                fileReader.readAsArrayBuffer(e.target.files[0]);

                fileReader.onload = (e) => {
                    //ArrayBuffer               　
                    const jsonDataBuffer = e.target.result;
                    //write to html
                    printJsonToHtmlDocument(jsonDataBuffer);
                };
            }
        });

        function printJsonToHtmlDocument(jsonBuffer) {

            const topDiv = document.createElement("div");
            topDiv.id = "topDivId";
            main.appendChild(topDiv);

            //decode
            const textData = new TextDecoder("utf-8").decode(jsonBuffer); 
            //parse json file
            const json = JSON.parse(textData);

            for(let i = 0; i < json.length; i++) {

                const div = document.createElement("div");
                div.className = `jsonContents`;

                //title
                const title = document.createElement("h2");
                title.style.margin = `${4}px`;
                title.style.borderBottom = "thin solid #566573";
                title.style.marginBottom = `${16}px`;
                title.textContent = `${i+1},  ${json[i].repo}`;
                title.id = `title_id_${i}`;

                //link
                const url = document.createElement("a");
                url.style.marginTop = `${8}px`;
                url.style.marginBottom = `${8}px`;
                url.style.marginLeft = `${56}px`;
                url.style.color = "#1da1f2";
                url.style.textDecoration = "none";
                url.target = "_blank";
                url.rel = "noopener";
                url.href = json[i].url;
                url.textContent = `🔗 ${json[i].url}`;
                url.id = `url_id_${i}`;

                //description
                const desc = document.createElement("p");
                desc.style.marginTop = `${8}px`;
                desc.style.marginBottom = `${8}px`;
                desc.style.marginLeft = `${56}px`;
                desc.textContent = `📖 ${json[i].desc}`;
                desc.id = `desc_id_${i}`;

                //star
                const star = document.createElement("p");
                star.style.marginTop = `${8}px`;
                star.style.marginBottom = `${8}px`;
                star.style.marginLeft = `${56}px`;
                star.textContent = `⭐ ${json[i].star}`;
                star.id = `star_id_${i}`;

                //keywords
                const keywords = document.createElement("p");
                keywords.style.marginTop = `${8}px`;
                keywords.style.marginBottom = `${8}px`;
                keywords.style.marginLeft = `${56}px`;          
                keywords.id = `keywords_id_${i}`;

                const keywordsArray = json[i].keywords.split(' ');
                let newArray = [];
                for(let j = 0; j < keywordsArray.length; j++) {
                    newArray.push(`[ ${keywordsArray[j]} ]`);
                }
                const regex = /,/g;
                keywords.textContent = `💎 ${newArray.toString().replace(regex, " ")}`;  

                topDiv.appendChild(div);
                div.appendChild(title);
                div.appendChild(url);
                div.appendChild(desc);
                div.appendChild(star);
                div.appendChild(keywords);
            } 
        }
    </script>
</body>
</html>
"###;
