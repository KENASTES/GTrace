using System;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Runtime.InteropServices;

namespace Front_End
{

    public partial class MainWindow : Window
    {
        [DllImport("core_engine.dll", CallingConvention = CallingConvention.Cdecl)]

        public static extern int process_garber_to_gcode(String garber_path);

        private string garber_selected_file_path = "";

        public MainWindow()
        {
            InitializeComponent();

            Generate_Button.Click += Generate_Button_Click;
        }

        private void Log_To_Console(string message)
        {
            Console_TextBox.AppendText(message + Environment.NewLine);
            Console_TextBox.ScrollToEnd();
        }

        private void Select_File_Button_Click(object sender, RoutedEventArgs e)
        {
            Open_File_Dialog open_File_Dialog = new Open_File_Dialog();

            open_File_Dialog.Filter = "Gerber Files (*.grb;*.gbr)|*.grb;*.gbr|All files (*.*)|*.*";

            if (open_File_Dialog.Show_Dialog() == true)
            {
                garber_selected_file_path = open_File_Dialog.FileName;

                File_Path_TextBox.Text = garber_selected_file_path;
                Log_To_Console("Selected file: " + garber_selected_file_path);
            }
        }

        private void Generate_Button_Click(object sender, RoutedEventArgs e)
        {
            if (String.IsNullOrEmpty(garber_selected_file_path))
            {
                Log_To_Console("Please select a Gerber file before generating G-code.");
                return;
            }

            string Feed_Rate_Setting_Dislay = Feed_Rate_Input.Text;
            string Laser_Power_Setting_Display = Laser_Pwoer_Input.Text;

            try
            {
                int Result = process_garber_to_gcode(garber_selected_file_path);

                if (Result == 0)
                {
                    Log_To_Console("G-code generation successful!");
                }
                else
                {
                    Log_To_Console("G-code generation failed with error code: " + Result);
                }
            }
            catch (Exception ex)
            {
                Log_To_Console("An error occurred during G-code generation: " + ex.Message);
            }
        }
    }

}
