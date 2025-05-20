## Solution

Online there are tools to manage pdfs (e.g. [`HandyOutliner`](https://handyoutlinerfo.sourceforge.net/)), but they are not automatised. Thus I would need a script which takes all pdf files in a folder, unites them, and create manually an index for the obtained file signing at which page does the n-th file start. The solution is not practical, I think it is thus better to write a tool which unites the files with `pdfunite` and index the output with `pdf.tocgen`. 

For the `extra` part I have an idea: gather the file names and pages, and then write a Markdown file with the index, and convert it to pdf with the usual `pandoc` or similar, and append it in the beginning of the document.


# only for me:

resource for a rust library to manipulate pdfs: https://github.com/J-F-Liu/lopdf
The readme of such includes links to **reference manuals**

Building the repo with such library would be much much more elegant and fast (in performance, not in results) I guess, rather than using external shell command line tools

I guess the best solution is to have a branch which makes almost a script, which uses xshell and calls pdfunite, pdf.tocgen etc..., and then AFTERWARDS develop a serious branch which uses `lopdf`