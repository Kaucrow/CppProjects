import re
import os
from date import get_date
from tqdm import tqdm, trange
from docx.api import Document
from lxml import etree
import time

def get_docx_data_1(document_path, data, keys):
    if not hasattr(get_docx_data_1, 'thesis_count'):
        get_docx_data_1.thesis_count = 0

    document = Document(document_path)
    table = document.tables[0]

    filename = os.path.splitext(os.path.basename(document_path))[0]
    period = filename[filename.find('-') + 1: ]

    for i, row in enumerate(table.rows[1: ]):
        text = []

        for cell in row.cells:
            cleaned_cell_text = (re.sub(r' +', ' ',
                                    '\n'.join(
                                        line.strip() for line in cell.text.split('\n'))
                                    )
                                ).strip()
            text.append(cleaned_cell_text)
        
        text.append(period)

        # Construct a dictionary for this row, mapping
        # keys to values for this row
        row_data = dict(zip(keys, text))

        data.append(row_data)
        get_docx_data_1.thesis_count += 1

        #pbar.update(1)
    
    #pbar.close()

def get_docx_data_2(document_path, data):
    if not hasattr(get_docx_data_2, 'thesis_count'):
        get_docx_data_2.thesis_count = 0

    document = Document(document_path)

    doc_xml = document.element.xml
    root = etree.fromstring(doc_xml)

    textboxes = root.xpath('//w:txbxContent', namespaces=root.nsmap)

    textbox_idx = 0

    paragraph_num = 0

    def check_idx(idx):
        if idx == -1:
            raise Exception(f'Error in main paragraph of thesis in document (delimiter not found): {document_path}')

    thesis = {}
    for paragraph in document.paragraphs:
        paragraph = paragraph.text.strip()
        if not paragraph == '':
            paragraph_num += 1
            match paragraph_num:
                case 1:
                    searchable = paragraph.lower()

                    # Thesis name
                    start_idx = searchable.find('especial:')
                    check_idx(start_idx)
                    start_idx += len('especial:')
                    end_idx = searchable.find('que', start_idx)
                    check_idx(end_idx)
                    thesis_name = paragraph[start_idx:end_idx].strip()
                    thesis['TITULO DE LA TESIS'] = thesis_name
                    searchable = searchable[end_idx:]
                    paragraph = paragraph[end_idx:]

                    # Student name
                    start_idx = searchable.find('bachiller:')
                    check_idx(start_idx)
                    start_idx += len('bachiller:')
                    end_idx = searchable.find('titular', start_idx)
                    check_idx(end_idx)
                    student = paragraph[start_idx:end_idx].strip()
                    thesis['ALUMNO'] = student
                    searchable = searchable[end_idx:]
                    paragraph = paragraph[end_idx:]

                    # C.I.
                    start_idx = searchable.find('v-')
                    check_idx(start_idx)
                    start_idx += len('v-')
                    substr = searchable[start_idx:]
                    match = re.search(r'[^0-9.]', substr.strip())
                    if match:
                        end_idx = match.start()
                    else:
                        check_idx(-1)
                    thesis['C.I.'] = substr[:end_idx + 1].strip()
                    searchable = searchable[end_idx:]
                    paragraph = paragraph[end_idx:]

                    # Grade
                    start_idx = searchable.find('(')
                    check_idx(start_idx)
                    start_idx += len('(')
                    end_idx = searchable.find(')')
                    check_idx(end_idx)
                    thesis['CALIFICACION'] = paragraph[start_idx:end_idx].strip()

                case 2:
                    start_idx = paragraph.find('a los')
                    check_idx(start_idx)
                    start_idx += len('a los')
                    end_idx = paragraph.find('.')
                    check_idx(end_idx)
                    thesis['FECHA DE DEFENSA'] = get_date(paragraph[start_idx:end_idx].strip())

                    teachers = []
                    ci_texts = []
                    older_ci = ''
                    last_ci = ''
                    repeated = False
                    for i in range(10):
                        textbox_texts = textboxes[textbox_idx + i].xpath('.//w:t/text()', namespaces=root.nsmap)
                        for text in textbox_texts:
                            if text[0].isdigit():
                                ci_texts.append(text)
                            elif ci_texts and text[0] == '.':
                                ci_texts.append(text)
                            elif ci_texts:
                                if last_ci:
                                    older_ci = last_ci
                                last_ci = ''.join(ci_texts)
                                if older_ci:
                                    if older_ci == last_ci:
                                        if not repeated:
                                            repeated = True
                                        else:
                                            raise Exception('Found three repeated textboxes')
                                    else:
                                        if repeated:
                                            repeated = False
                                        else:
                                            raise Exception('Found no textbox repetition')

                                ci_texts = []

                            if repeated == False:
                                if text == 'TUTOR':
                                    teachers.insert(0, last_ci)
                                elif text == 'JURADO':
                                    teachers.append(last_ci)

                    textbox_idx += 10
                    paragraph_num = 0

                    thesis['JURADO PRINCIPAL'] = teachers
                    data.append(thesis)

                    get_docx_data_2.thesis_count += 1

                    thesis = {}