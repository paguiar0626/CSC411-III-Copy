# **Specification**
*	Array2 must be implemented as a polymorphic structure with the following qualities:
** Be able to store multiple types of data (e.g., u32, &Str, String, etc.).
** u32 or above, being the most probably for image storage.
** Represent 2-dimensional data in either 2d or 1d implementation.
**	Uses Vec<T>.
**	Implement iterator.
**	To be used by another function or program.
# **Design**
• Data Structure
*-	A public structure named “Array2” of T type.
**▪	Its 3 properties are:
**▪	data (Vec<T>)
**▪	width (usize)
**▪	height (usize)
*-	Public methods are:
**▪	Constructor (“new”) with arguments of width: usize and height: usize.
***○	New instances of Array2 must have known dimensions.
***○	Width and height are assigned argument values.
***○	The vector data is then assigned a vector of width * height dimensions.
**▪	Insert method with arguments of row: usize, col: usize, and value: T.
***○	Values for row and col must be lower than instance’s width and height respectively.
***○	Store incoming values in a 1d representation (i.e., just a Vec<T>).
***○	An index value will be tracked with the formula:
****◉	row * self.width + col
***○	Assign “data[index]” the value argument.
*-	“iter_row_major” method that returns a standard implementation of an iterator
**▪	-> impl Iterator<Item = &T>
**▪	Returns “self.data.iter()”
• Invariants
*-	Data entered into Array2 will be 2 dimensional (width * height)
*-	Declaration of an empty Array2 must know and provide the dimension numbers for row and column.
*-	Array2 vector will use 1 dimensional representation by using the index formula of Row * Width + Column.
*-	As such, invalid data will be any data that is not the correct dimensions or not of one type (e.g., u32, &Str, and etc.).
*-	In 
• How does this let me solve the problem?	
*-	Given our plans, the 2d data (such as the Sudoku grid) would be able to be inserted into an empty Array2 object by providing three arguments (row, column numbers and data value) to the insert method. Other programs and functions can then iteratively access the contents of an Array2 object.
# **Testing**
In terms of testing, the structure is able to take arguments for width and height and assigning to their same-named attributes. The rest of the plans needs further testing. Iterator syntax was adopted from lecture notes.
