# 📌 Smart Attendance System - Workflow

## 1️⃣ Landing Page

Users are presented with two options:

- **[Login]** → For existing users.
- **[Register]** → For new users.

---

## 2️⃣ Registration Flow (Account Creation First)

Users must create an account **before** choosing to create or join an organization.

### **Step 1: User Registration**

1. User enters:
   - **Full Name**
   - **Email or Phone Number** (used as a unique identifier)
   - **Password**
2. System checks:
   - Is the email/phone already registered?
   - Does the password meet security requirements?
3. If valid, store user data and redirect to the **Organization Selection** page.

---

## 3️⃣ Organization Selection (After Registration)

Once registered, users must decide:

- **[Create Organization]** → To start a new organization.
- **[Join Organization]** → To join an existing organization.
- **[My Organizations]** → If they already belong to an organization.

---

## 4️⃣ Create Organization Flow

Only users with a valid **subscription plan** can create multiple organizations.

1. User enters:
   - **Organization Name**
   - **Organization Email or Phone Number**
   - **Organization Password** (for security)
   - **Subscription Plan** (Only required if creating more than one organization)
2. System checks:
   - Does the organization already exist?
   - Does the user have a valid subscription for multiple organizations?
3. If valid, create the organization and assign the user as **Admin**.
4. Redirect to **Admin Dashboard**.

---

## 5️⃣ Join Organization Flow

Users who want to **join an existing organization** must provide:

1. **Personal Email or Phone Number**
2. **Organization Email or Phone Number**
3. **Organization Password** (for verification)
4. **Password**

### **Validation Checks:**

- Does the organization exist?
- Is the organization password correct?
- Is the user already in the organization?
- (Optional) Does an admin need to approve the request?

If valid, assign the appropriate role:

- **Staff** → Can mark attendance, view reports, and limited settings.
- **User** → Can only view their own attendance.

Redirect to **Dashboard**.

---

## 6️⃣ Post-Login Role-Based Access

### **Admin Role**

- Can **add/manage users and staff**.
- Can **view and edit attendance records**.
- Can **manage organization settings**.

### **Staff Role**

- Can **mark attendance**.
- Can **view attendance reports**.
- Cannot manage users or organization settings.

### **User Role**

- Can **only view their own attendance records**.

---

## 7️⃣ Logout

Users can log out at any time, returning to the **Landing Page**.
