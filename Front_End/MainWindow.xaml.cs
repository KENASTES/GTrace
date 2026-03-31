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

        public static extern int add_test(int a, int b);
        public MainWindow()
        {
            InitializeComponent();

            TestRustConnection();
        }

        private void TestRustConnection()
        {
            try
            {
                int result = add_test(50,50);
                MessageBox.Show($"ทดสอบเชื่อมต่อ GTrace Core สำเร็จ!\nผลลัพธ์จาก Rust (50 + 50) = {result}", "FFI Test");
            }
            catch (Exception ex)
            {
                MessageBox.Show($"หาไฟล์ DLL ไม่เจอ หรือเกิดข้อผิดพลาด:\n{ex.Message}", "FFI Error");
            }
        }
    }

}
