# Code của Nhánh 1: Đảm nhận phần hiển thị thông tin

def hien_thi_thong_tin(sinh_vien):
    """
    Hàm này nhận vào một đối tượng sinh viên (được tạo từ Class của nhánh 2 và 3)
    và in thông tin của sinh viên đó ra màn hình.
    """
    print("=" * 40)
    print("      KẾT QUẢ KIỂM TRA QUY TRÌNH GIT      ")
    print("=" * 40)
    print(f"📌 Tên sinh viên: {sinh_vien.ten}")
    print(f"📌 Mã số sinh viên: {sinh_vien.msv}")
    print(f"📌 Chuyên ngành: {sinh_vien.chuyen_nganh}")
    print("=" * 40)
    print("🎉 Chúc mừng nhóm bạn! Gộp nhánh thành công 100%!")
