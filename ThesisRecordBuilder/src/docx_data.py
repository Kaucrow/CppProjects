import re
import os
from tqdm import tqdm, trange
from docx.api import Document

def get_docx_data(document_path, data, keys):
    if not hasattr(get_docx_data, 'thesis_count'):
        get_docx_data.thesis_count = 0;

    document = Document(document_path);
    table = document.tables[0];

    filename = os.path.splitext(os.path.basename(document_path))[0];
    period = filename[filename.find('-') + 1: ];

    #pbar = tqdm(total = len(table.rows[1: ]), position=1, leave=False, bar_format='Row: {n}/{total} [{percentage:0.0f}%]')

    for i, row in enumerate(table.rows[1: ]):
        text = [];
    
        for cell in row.cells:
            cleaned_cell_text = (re.sub(r' +', ' ',
                                    '\n'.join(
                                        line.strip() for line in cell.text.split('\n'))
                                    )
                                ).strip();
            text.append(cleaned_cell_text);
        
        text.append(period);

        # Construct a dictionary for this row, mapping
        # keys to values for this row
        row_data = dict(zip(keys, text))

        data.append(row_data);
        get_docx_data.thesis_count += 1; 

        #pbar.update(1)
    
    #pbar.close()