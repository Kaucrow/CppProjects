import sys
import getopt
import os
import csv
from tqdm import tqdm, trange
from format_copy import *
from docx_data import *
from verdicts_data import get_verdicts_data, display
from docx.api import Document

def clear():
    if os.name == 'nt':
        _ = os.system('cls')
    else:
        _ = os.system('clear')

clear()

class Globals:
    DEBUG: bool = False
    DATA_FOLDER: str = 'data'
    KNOWN_MENTIONS = ['HONORIFICA', 'PUBLICACION']

global globals
globals = Globals()

try:
    opts, args = getopt.getopt(sys.argv[1:], 'ehd', ['example', 'help', 'debug'])
except getopt.GetoptError as err:
    print(err)
    exit(1)

for opt, arg in opts:
    if opt in ('-h', '--help'):
        print(
            '\n'.join(
                line.strip() for line in
                """
                [-h, --help]: Displays this help screen.
                [-d, --debug]: Runs the program in debug mode.
                [-e, --example]: Runs the program using the example files in the `data_ex` directory.
                """.split('\n')
            )
        )
        exit(0)

    if opt in ('-e', '--example'):
        globals.DATA_FOLDER = 'data_ex'

    if opt in ('-d', '--debug'):
        globals.DEBUG = True

thesis_list = []
teachers_names = {}
teachers_data = {}

with open(globals.DATA_FOLDER + '/in/teachers/teachers.csv', encoding='utf-8') as teachers_file:
    csv_reader = csv.reader(teachers_file, delimiter=';');
    line_count = 0;
    for row in csv_reader:
        if line_count < 2:
            line_count += 1;
            continue;
        else:
            base_name = row[1].split('|')[0];
            for teacher in row[1].split('|'):
                teachers_names[teacher] = base_name;
            filename = base_name.replace(' ', '_').upper();
            teachers_data[base_name] = {'FILENAME':filename, 'FULL NAME':row[3] + ' ' + row[0], 'C.I.':row[2], 'THESIS':[]};

keys = [
    'No', 'ALUMNO', 'C.I.', 'FECHA DE DEFENSA', 'TITULO DE LA TESIS', 'JURADO PRINCIPAL',
    'JURADO SUPLENTE', 'HORA', 'PERIODO', 'CALIFICACION', 'MENCION'
]

calendars_path = globals.DATA_FOLDER + '/in/calendars/'
calendar_files = os.listdir(calendars_path)

print('Collecting data...')

with tqdm(total=len(calendar_files), leave = True) as pbar_cal:
    for calendar in calendar_files:
        pbar_cal.set_description(f"Calendar: {calendar}")
        get_docx_data(calendars_path + calendar, thesis_list, keys)
        pbar_cal.update(1)

verdicts_path = globals.DATA_FOLDER + '/in/verdicts/'
verdict_files = os.listdir(verdicts_path)

print()

with tqdm(total=len(verdict_files), leave = True) as pbar_vrd:
    for verdict in verdict_files:
        pbar_vrd.set_description(f"Verdict: {os.path.basename(verdict)}")
        try:
            get_verdicts_data(verdicts_path + verdict, thesis_list, globals)
        except Exception as err:
            print('[ ERR ] Error occurred when processing verdict: ' + verdicts_path + verdict + '\n\terr: ' + err.args[0])
            print('\tTesseract text:\n\n' + err.args[3])
            display(err.args[1])
            display(err.args[2])
            exit(1)
        pbar_vrd.update(1)

dest_document = Document();
filename = '';
curr_period = None

date = input("Elaboration date (for footer): ");

for i in range(3):
    for idx, thesis in enumerate(thesis_list):
        teacher_found_name = thesis['JURADO PRINCIPAL'].split('\n')[i].replace('Arq.', '').strip();
        if teacher_found_name in teachers_names:
            teacher_real_name = teachers_names[teacher_found_name];
            if i == 0:
                teachers_data[teacher_real_name]['THESIS'].append({'IDX':idx, 'TYPE':'tutor'});
            else:
                teachers_data[teacher_real_name]['THESIS'].append({'IDX':idx, 'TYPE':'jury'});
        else:
            sys.exit('Could not find real name of teacher \'' + teacher_found_name + '\' (From thesis of period ' + thesis['PERIODO'] + ').\nPlease update the `teachers.csv` file');

for teacher, info in teachers_data.items():
    info['THESIS'].sort(key = lambda x: thesis_list[x['IDX']]['PERIODO']);

tutor_curr_period = "";
jury_curr_period = "";
tutor_doc = Document();
jury_doc = Document();
p = None;

for teacher, teacher_info in teachers_data.items():
    tutor_curr_period = "";
    jury_curr_period = "";
    for thesis in teacher_info['THESIS']:
        thesis_data = thesis_list[thesis['IDX']];
        if thesis['TYPE'] == 'tutor':
            if tutor_curr_period == "":
                tutor_doc = Document();
                copy_header(globals.DATA_FOLDER, tutor_doc, teacher_info['FULL NAME'], teacher_info['C.I.'], 'TUTOR');
            
            if thesis_data['PERIODO'] != tutor_curr_period:
                tutor_curr_period = thesis_data['PERIODO'];
                p = tutor_doc.add_paragraph();
                p.style.font.name = 'Arial';
                p.style.font.size = Pt(11);
                p.paragraph_format.space_after = Pt(20);

                period_run = p.add_run('PERIODO ACADÉMICO ' + tutor_curr_period);
                period_run.bold = True;
                period_run.italic = True;

            p = tutor_doc.add_paragraph();

        elif thesis['TYPE'] == 'jury':
            if jury_curr_period == "":
                jury_doc = Document();
                copy_header(globals.DATA_FOLDER, jury_doc, teacher_info['FULL NAME'], teacher_info['C.I.'], 'JURADO');
            
            if thesis_data['PERIODO'] != jury_curr_period:
                jury_curr_period = thesis_data['PERIODO'];
                p = jury_doc.add_paragraph();
                p.style.font.name = 'Arial';
                p.style.font.size = Pt(11);
                p.paragraph_format.space_after = Pt(20);

                period_run = p.add_run('PERIODO ACADÉMICO ' + jury_curr_period);
                period_run.bold = True;
                period_run.italic = True;
            
            p = jury_doc.add_paragraph();

        else:
            sys.exit('[ ERR ] Found unknown thesis type \'' + thesis['TYPE'] + '\' in teacher \'' + teacher + '\' data.');

        p.style.font.name = 'Arial';
        p.style.font.size = Pt(11);
        p.paragraph_format.space_after = Pt(20);

        p.add_run('Nombre: ').italic = True;
        name_run = p.add_run(thesis_data.get('ALUMNO').split('\n')[0].upper() + '\n');
        name_run.bold = True;
        name_run.italic = True;

        p.add_run('Titulo de T.E.G: ').italic = True;
        title_run = p.add_run(' '.join(line.strip() for line in thesis_data.get('TITULO DE LA TESIS').split('\n')).upper());
        title_run.bold = True;
        title_run.italic = True;

    if tutor_curr_period != "":
        copy_footer(globals.DATA_FOLDER, tutor_doc, date);
        tutor_doc.save(globals.DATA_FOLDER + '/out/CONSTANCIA_TUTOR_' + teacher_info['FILENAME'] + '.docx');

    if jury_curr_period != "":
        copy_footer(globals.DATA_FOLDER, jury_doc, date);
        jury_doc.save(globals.DATA_FOLDER + '/out/CONSTANCIA_JURADO_' + teacher_info['FILENAME'] + '.docx');

"""
for i in range(3):
    thesis_list.sort(key = lambda x: x['JURADO PRINCIPAL'].split('\n')[i]);
    for thesis in thesis_list:
        curr_teacher = thesis['JURADO PRINCIPAL'].split('\n')[i];
        
        if curr_teacher not in known_teachers:
            copy_footer(dest_document, date);

            if i == 0:
                dest_document.save('data/out/CONSTANCIA_TUTOR_' + filename + '.docx');
            else:
                dest_document.save('data/out/CONSTANCIA_JURADO_' + filename + '.docx');

            print('No teacher named \'' + curr_teacher + '\' (Found in period ' + thesis['PERIODO'] + ')');
            filename = input('* Filename: ');
            teacher_name = input('* Teacher name: ');
            teacher_ci = input('* Teacher ci: ');
            known_teachers[curr_teacher] = (filename, teacher_name, teacher_ci);
            dest_document = Document();

            if i == 0:
                copy_header(dest_document, teacher_name, teacher_ci, 'TUTOR');
            else:
                copy_header(dest_document, teacher_name, teacher_ci, 'JURADO');
        
        if thesis['PERIODO'] != curr_period:
            curr_period = thesis['PERIODO'];
            p = dest_document.add_paragraph();
            p.style.font.name = 'Arial';
            p.style.font.size = Pt(11);
            p.paragraph_format.space_after = Pt(20);
        
            period_run = p.add_run('PERIODO ACADÉMICO ' + curr_period);
            period_run.bold = True;
            period_run.italic = True;

        p = dest_document.add_paragraph();
        p.style.font.name = 'Arial';
        p.style.font.size = Pt(11);
        p.paragraph_format.space_after = Pt(20);

        p.add_run('Nombre: ').italic = True;
        name_run = p.add_run(thesis.get('ALUMNO').split('\n')[0].upper() + '\n');
        name_run.bold = True;
        name_run.italic = True;

        p.add_run('Titulo de T.E.G: ').italic = True;
        title_run = p.add_run(' '.join(line.strip() for line in thesis.get('TITULO DE LA TESIS').split('\n')).upper());
        title_run.bold = True;
        title_run.italic = True;
    
copy_footer(dest_document);
dest_document.save('data/out/CONSTANCIA_JURADO_' + filename + '.docx');
"""
print("Finished execution.");