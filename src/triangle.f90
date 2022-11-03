function area(name, name_length, A, B) result(S) bind(C, name="area")
  ! area of a triangle
  use iso_c_binding, only: c_char, c_size_t, c_float, c_null_char
  implicit none
  ! name
  integer(kind=c_size_t), intent(in), value :: name_length
  character(kind=c_char), dimension(name_length), intent(in) :: name
  character(len=name_length) :: fname
  integer(kind=c_size_t) :: i

  ! area
  real(kind=c_float), intent(in), value :: A, B
  real(kind=c_float) :: S

  do i=1,name_length
     fname(i:i) = name(i)
  end do

  S = (A * B)/2

  write(*,*) name, ", area =", S
  write(*,*) fname, ", area =", S
end function area
