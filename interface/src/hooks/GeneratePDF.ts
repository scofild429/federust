
import JsPDF from 'jspdf'
import html2canvas from 'html2canvas'

/////orignal
// const generatePDF = (dom: HTMLElement) => {
// 	const title = 'protocol_in_steps'
// 	html2canvas(dom).then(function (canvas) {
// 		const contentWidth = canvas.width
// 		const contentHeight = canvas.height
// 		const pdfWidth = (contentWidth + 10) / 2 * 0.75
// 		const imgWidth = pdfWidth
// 		const imgHeight = contentHeight / 2 * 0.75
// 		const pageData = canvas.toDataURL('image/jpeg', 1.0)
// 		const pdf = new JsPDF('p', 'pt', 'a4')
// 		pdf.addImage(pageData, 'jpeg', 50, 50, imgWidth, imgHeight)
// 		pdf.save(title + '.pdf')
// 	})
// }


// works with multiple page, but without margin for each page
// const generatePDF = (dom: HTMLElement) => {
// 	const title = 'protocol_in_steps'
// 	html2canvas(dom).then(function (canvas) {
// 	  const imgWidth = 210;
// 	  const pageHeight = 295;
// 	  const imgHeight = (canvas.height * imgWidth) / canvas.width;
// 	  let heightLeft = imgHeight;
// 	  let margin = 20;
// 	  let position = 20;
// 	  heightLeft -= pageHeight;
// 	  const doc = new JsPDF('p', 'mm');
// 	  doc.addImage(canvas, 'PNG', margin, position, imgWidth - (margin * 2), imgHeight - (margin * 2), '', 'FAST');
// 	  while (heightLeft >= 0) {
// 	    position = heightLeft - imgHeight;
// 	    doc.addPage();
// 	    doc.addImage(canvas, 'PNG', margin, position, imgWidth - (margin * 2), imgHeight - (margin * 2), '', 'FAST');
// 	    heightLeft -= pageHeight;
// 	  }
// 	  doc.save(title+'.pdf');
// 	})
// }


const generatePDF = (dom: HTMLElement) => {

  html2canvas(dom, { useCORS: true, allowTaint: true, scrollY: 0 }).then(function (canvas) {
    const image = { type: 'jpeg', quality: 0.98 };
    const margin = [0.5, 0.5];
    const filename = 'protocol_in_steps.pdf';

    var imgWidth = 8.5;
    var pageHeight = 11;

    var innerPageWidth = imgWidth - margin[0] * 2;
    var innerPageHeight = pageHeight - margin[1] * 2;

    // Calculate the number of pages.
    var pxFullHeight = canvas.height;
    var pxPageHeight = Math.floor(canvas.width * (pageHeight / imgWidth));
    var nPages = Math.ceil(pxFullHeight / pxPageHeight);

    // Define pageHeight separately so it can be trimmed on the final page.
    var pageHeight = innerPageHeight;

    // Create a one-page canvas to split up the full image.
    var pageCanvas = document.createElement('canvas');
    var pageCtx = pageCanvas.getContext('2d');
    pageCanvas.width = canvas.width;
    pageCanvas.height = pxPageHeight;

    // Initialize the PDF.
    var pdf = new JsPDF('p', 'in', [8.5, 11]);

    for (var page = 0; page < nPages; page++) {
      // Trim the final page to reduce file size.
      if (page === nPages - 1 && pxFullHeight % pxPageHeight !== 0) {
        pageCanvas.height = pxFullHeight % pxPageHeight;
        pageHeight = (pageCanvas.height * innerPageWidth) / pageCanvas.width;
      }

      // Display the page.
      var w = pageCanvas.width;
      var h = pageCanvas.height;
      pageCtx!.fillStyle = 'white';
      pageCtx!.fillRect(0, 0, w, h);
      pageCtx!.drawImage(canvas, 0, page * pxPageHeight, w, h, 0, 0, w, h);

      // Add the page to the PDF.
      if (page > 0) pdf.addPage();
      var imgData = pageCanvas.toDataURL('image/' + image.type, image.quality);
      pdf.addImage(imgData, image.type, margin[1], margin[0], innerPageWidth, pageHeight);
    }

    pdf.save(filename);
    
  })
}

export default generatePDF
